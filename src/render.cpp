#include "render.hpp"

void	render::renderMenu(App &app)
{
	SDL_RenderClear(app.renderer);

	renderSprite(app.renderer, app.backgroundTexture, NULL);

}

void	render::renderSprite(SDL_Renderer *renderer, SDL_Texture *texture, const SDL_FRect* dst)
{
	SDL_RenderTexture(renderer, texture, NULL, dst);
}
