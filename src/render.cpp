#include <cstddef>

#include "render.hpp"

static void	renderText(SDL_Renderer* renderer, TTF_Font* font,
					   const std::string& text, const SDL_FRect& rect,
					   SDL_Color color)
{
	SDL_Surface* surface = TTF_RenderText_Blended(font, text.c_str(), 0, color);
    if (!surface)
        return ;

    SDL_Texture* texture = SDL_CreateTextureFromSurface(renderer, surface);
    SDL_DestroySurface(surface);
    if (!texture)
        return ;

    float tw, th;
    SDL_GetTextureSize(texture, &tw, &th);

    SDL_FRect dst = {
        rect.x + (rect.w - tw) / 2.0f,
        rect.y + (rect.h - th) / 2.0f,
        tw,
        th
    };

    SDL_RenderTexture(renderer, texture, NULL, &dst);
    SDL_DestroyTexture(texture);

}

void	render::renderMenu(App &app)
{
	SDL_RenderClear(app.renderer);
	renderSprite(app.renderer, app.backgroundTexture, NULL);

	for (std::size_t i = 0; i < app.mainMenu.items.size(); i++)
	{
		const MenuItem&	item = app.mainMenu.items[i];
		bool			selected = (static_cast<int>(i) == app.mainMenu.curr);

		if (selected)
			SDL_SetRenderDrawColor(app.renderer, 255, 215, 0, 255);
		else
			SDL_SetRenderDrawColor(app.renderer, 50, 50, 50, 200);

		SDL_RenderFillRect(app.renderer, &item.rect);

		SDL_SetRenderDrawColor(app.renderer, 255, 255, 255, 255);
		SDL_RenderRect(app.renderer, &item.rect);

		SDL_Color	textColor = {0, 0, 0, 255};
		if (!selected)
			textColor = {255, 255, 255, 255};

		renderText(app.renderer, app.font, item.label, item.rect, textColor);
	}
}

void	render::renderSprite(SDL_Renderer *renderer, SDL_Texture *texture, const SDL_FRect* dst)
{
	SDL_RenderTexture(renderer, texture, NULL, dst);
}
