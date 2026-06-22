#pragma once

#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>

class AppState {
	public:
		SDL_Window*		window;
		SDL_Renderer*	renderer;
		bool			keys[SDL_SCANCODE_COUNT];

		AppState();
		AppState(const AppState& other) = default;
		AppState& operator=(const AppState& other) = default;
		~AppState() = default;

	private:
};
