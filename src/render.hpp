#pragma once

#include "App.hpp"

namespace render {

	void	renderMenu(App& app);

	void	renderSprite(SDL_Renderer* renderer, SDL_Texture* texture, const SDL_FRect* dst);

}
