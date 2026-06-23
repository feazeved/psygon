#pragma once

#include "AppState.hpp"

namespace render {

	void	renderMenu(AppState& app);

	void	renderSprite(SDL_Renderer* renderer, SDL_Texture* texture, const SDL_FRect* dst);

}
