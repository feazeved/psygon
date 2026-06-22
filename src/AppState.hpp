#pragma once

#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>

enum class GameState
{
	STATE_MENU,
	STATE_PLAYING,
	STATE_SETTINGS,
	STATE_QUIT
};

class AppState {
	public:
		SDL_Window*		window;
		SDL_Renderer*	renderer;
		GameState		state;
		bool			keys[SDL_SCANCODE_COUNT];
		SDL_Texture*	backgroundTexture;


		AppState();
		AppState(const AppState& other) = default;
		AppState& operator=(const AppState& other) = default;
		~AppState() = default;

	private:
};
