#include <stdlib.h>

#include <SDL2/SDL.h>
#include <string>

class Screen {
  public:
    Screen(std::string title, int width, int height) {
      SDL_Init(SDL_INIT_VIDEO);
      SDL_CreateWindowAndRenderer(width, height, 0, &window, &renderer);
      SDL_SetWindowTitle(window, title.c_str());
      SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0);
      SDL_RenderClear(renderer);
    }

    ~Screen() {
      SDL_DestroyRenderer(renderer);
      SDL_DestroyWindow(window);
      SDL_Quit();
    }

    void close() {
      SDL_Event event;
      while (1) {
        if (SDL_PollEvent(&event) && event.type == SDL_QUIT)
          break;
      }
    }

    void drawPixel(int x, int y) {
      SDL_SetRenderDrawColor(renderer, 255, 255, 255, 0);
      SDL_RenderDrawPoint(renderer, x, y);
      SDL_RenderPresent(renderer);
    }

    void drawLine(int x1, int y1, int x2, int y2) {
      int dx = abs(x2 - x1), sx = x1 < x2 ? 1 : -1;
      int dy = abs(y2 - y1), sy = y1 < y2 ? 1 : -1;
      int err = (dx > dy ? dx : -dy) / 2;

      while (true) {
        drawPixel(x1, y1);
        if (x1 == x2 && y1 == y2) break;
        int e2 = err;
        if (e2 > -dx) { err -= dy; x1 += sx; }
        if (e2 < dy) { err += dx; y1 += sy; }
      }
    }

    void clearScreen() {
      SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0);
      SDL_RenderClear(renderer);
    }

  private:
    SDL_Renderer *renderer;
    SDL_Window *window;
};

int main(void) {
    Screen screen("Smiya Rendering Engine", 600, 600);
    for(int i = 0; i < 900; i++){
      screen.drawLine(300, 300, 200, i);
      screen.clearScreen();
    }
    screen.close();
    return 0;
}
