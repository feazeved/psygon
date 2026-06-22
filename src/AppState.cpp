#include "AppState.hpp"

AppState::AppState() :
	window(nullptr), renderer(nullptr)
{
	for (size_t i = 0; i < SDL_SCANCODE_COUNT; i++)
		keys[i] = false;
}
