/**
 * @file pager.c
 * @author Joe Wingbermuehle
 * @date 2004-2006
 *
 * @brief Pager tray component.
 *
 */

#include "jwm.h"
#include "pager.h"
#include "tray.h"
#include "main.h"
#include "desktop.h"
#include "client.h"
#include "color.h"

typedef struct PagerType {

   TrayComponentType *cp;

   int deskWidth;
   int deskHeight;
   double scalex, scaley;
   LayoutType layout;

   Pixmap buffer;

   struct PagerType *next;

} PagerType;

static PagerType *pagers;

static void Create(TrayComponentType *cp);

static void SetSize(TrayComponentType *cp, int width, int height);

static void ProcessPagerButtonEvent(TrayComponentType *cp,
   int x, int y, int mask);

static void DrawPagerClient(const PagerType *pp, const ClientNode *np);

/** Initialize pager data. */
void InitializePager() {
   pagers = NULL;
}

/** Startup the pager. */
void StartupPager() {
}

/** Shutdown the pager. */
void ShutdownPager() {

   PagerType *pp;

   for(pp = pagers; pp; pp = pp->next) {
      JXFreePixmap(display, pp->buffer);
   }

}

/** Release pager data. */
void DestroyPager() {

   PagerType *pp;

   while(pagers) {
      pp = pagers->next;
      Release(pagers);
      pagers = pp;
   }

}

/** Create a new pager tray component. */
TrayComponentType *CreatePager() {

   TrayComponentType *cp;
   PagerType *pp;

   pp = Allocate(sizeof(PagerType));
   pp->next = pagers;
   pagers = pp;

   cp = CreateTrayComponent();
   cp->object = pp;
   pp->cp = cp;
   cp->Create = Create;
   cp->SetSize = SetSize;
   cp->ProcessButtonEvent = ProcessPagerButtonEvent;

   return cp;
}

/** Initialize a pager tray component. */
void Create(TrayComponentType *cp) {

   PagerType *pp;

   Assert(cp);

   pp = (PagerType*)cp->object;

   Assert(pp);

   Assert(cp->width > 0);
   Assert(cp->height > 0);

   cp->pixmap = JXCreatePixmap(display, rootWindow, cp->width,
      cp->height, rootDepth);
   pp->buffer = cp->pixmap;

}

/** Set the size of a pager tray component. */
void SetSize(TrayComponentType *cp, int width, int height) {

   PagerType *pp;

   Assert(cp);

   pp = (PagerType*)cp->object;

   Assert(pp);

   if(width) {

      /* Vertical pager, compute height from width. */
      cp->width = width;
      pp->deskWidth = width;
      pp->deskHeight = (cp->width * rootHeight) / rootWidth;
      cp->height = (pp->deskHeight + 1) * desktopCount;
      pp->layout = LAYOUT_VERTICAL;

   } else if(height) {

      /* Horizontal pager, compute width from height. */
      cp->height = height;
      pp->deskHeight = height;
      pp->deskWidth = (cp->height * rootWidth) / rootHeight;
      cp->width = (pp->deskWidth + 1) * desktopCount;
      pp->layout = LAYOUT_HORIZONTAL;

   } else {
      Assert(0);
   }

   pp->scalex = (double)(pp->deskWidth - 2) / rootWidth;
   pp->scaley = (double)(pp->deskHeight - 2) / rootHeight;

}

/** Process a button event on a tray component. */
void ProcessPagerButtonEvent(TrayComponentType *cp, int x, int y, int mask) {

   PagerType *pp;

   switch(mask) {
   case Button1:
   case Button2:
   case Button3:
      pp = (PagerType*)cp->object;
      if(pp->layout == LAYOUT_HORIZONTAL) {
         ChangeDesktop(x / (pp->deskWidth + 1));
      } else {
         ChangeDesktop(y / (pp->deskHeight + 1));
      }
      break;
   case Button4:
      PreviousDesktop();
      break;
   case Button5:
      NextDesktop();
      break;
   default:
      break;
   }
}

/** Update the pager. */
void UpdatePager() {

   PagerType *pp;
   ClientNode *np;
   Pixmap buffer;
   int width, height;
   int deskWidth, deskHeight;
   unsigned int x;

   if(shouldExit) {
      return;
   }

   for(pp = pagers; pp; pp = pp->next) {

      buffer = pp->cp->pixmap;
      width = pp->cp->width;
      height = pp->cp->height;
      deskWidth = pp->deskWidth;
      deskHeight = pp->deskHeight;

      /* Draw the background. */
      JXSetForeground(display, rootGC, colors[COLOR_PAGER_BG]);
      JXFillRectangle(display, buffer, rootGC, 0, 0, width, height);

      /* Highlight the current desktop. */
      JXSetForeground(display, rootGC, colors[COLOR_PAGER_ACTIVE_BG]);
      if(pp->layout == LAYOUT_HORIZONTAL) {
         JXFillRectangle(display, buffer, rootGC,
            currentDesktop * (deskWidth + 1), 0,
            deskWidth, height);
      } else {
         JXFillRectangle(display, buffer, rootGC,
            0, currentDesktop * (deskHeight + 1),
            width, deskHeight);
      }

      /* Draw the clients. */
      for(x = LAYER_BOTTOM; x <= LAYER_TOP; x++) {
         for(np = nodeTail[x]; np; np = np->prev) {
            DrawPagerClient(pp, np);
         }
      }

      /* Draw the desktop dividers. */
      JXSetForeground(display, rootGC, colors[COLOR_PAGER_FG]);
      for(x = 1; x < desktopCount; x++) {
         if(pp->layout == LAYOUT_HORIZONTAL) {
            JXDrawLine(display, buffer, rootGC,
               (deskWidth + 1) * x - 1, 0,
               (deskWidth + 1) * x - 1, height);
         } else {
            JXDrawLine(display, buffer, rootGC,
               0, (deskHeight + 1) * x - 1,
               width, (deskHeight + 1) * x - 1);
         }
      }

      /* Tell the tray to redraw. */
      UpdateSpecificTray(pp->cp->tray, pp->cp);

   }

}

/** Draw a client on the pager. */
void DrawPagerClient(const PagerType *pp, const ClientNode *np) {

   int x, y;
   int width, height;
   int deskOffset;
   ColorType fillColor;

   if(!(np->state.status & STAT_MAPPED)) {
      return;
   }

   if(np->state.status & STAT_STICKY) {
      deskOffset = currentDesktop;
   } else {
      deskOffset = np->state.desktop;
   }
   if(pp->layout == LAYOUT_HORIZONTAL) {
      deskOffset *= pp->deskWidth + 1;
   } else {
      deskOffset *= pp->deskHeight + 1;
   }

   x = (int)((double)np->x * pp->scalex + 1.0);
   y = (int)((double)np->y * pp->scaley + 1.0);
   width = (int)((double)np->width * pp->scalex);
   height = (int)((double)np->height * pp->scaley);

   if(x + width > pp->deskWidth) {
      width = pp->deskWidth - x;
   }
   if(y + height > pp->deskHeight) {
      height = pp->deskHeight - y;
   }
   if(x < 0) {
      width += x;
      x = 0;
   }
   if(y < 0) {
      height += y;
      y = 0;
   }
   if(width <= 0 || height <= 0) {
      return;
   }

   if(pp->layout == LAYOUT_HORIZONTAL) {
      x += deskOffset;
   } else {
      y += deskOffset;
   }

   JXSetForeground(display, rootGC, colors[COLOR_PAGER_OUTLINE]);
   JXDrawRectangle(display, pp->cp->pixmap, rootGC, x, y, width, height);

   if(width > 1 && height > 1) {
      if((np->state.status & STAT_ACTIVE)
         && (np->state.desktop == currentDesktop
         || (np->state.status & STAT_STICKY))) {
         fillColor = COLOR_PAGER_ACTIVE_FG;
      } else {
         fillColor = COLOR_PAGER_FG;
      }
      JXSetForeground(display, rootGC, colors[fillColor]);
      JXFillRectangle(display, pp->cp->pixmap, rootGC, x + 1, y + 1,
         width - 1, height - 1);
   }

}

