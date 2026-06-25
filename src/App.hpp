#pragma once

#include <string>
#include <exception>

#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>

enum class GameState
{
	STATE_MENU,
	STATE_PLAYING,
	STATE_SETTINGS,
	STATE_QUIT
};

class App {
	public:
		SDL_Window*		window;
		SDL_Renderer*	renderer;
		GameState		state;
		SDL_Texture*	backgroundTexture;
		const bool*		keys;

		App();
		App(const App& other) = default;
		App& operator=(const App& other) = default;
		~App();

	private:
		static const int	winWidth;
		static const int	winHeight;

	public:
		class InitException : public std::exception {
			public:
				InitException(const std::string& where);
				virtual const char* what() const noexcept override;
			private:
				std::string	message;
		};
};
