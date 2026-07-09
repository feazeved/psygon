#pragma once

#include <string>
#include <exception>
#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>
#include <SDL3_ttf/SDL_ttf.h>

#include "Menu.hpp"
#include "GameState.hpp"

class App {
	public:
		SDL_Window*		window;
		SDL_Renderer*	renderer;
		SDL_Texture*	backgroundTexture;
		Menu			mainMenu;
		TTF_Font*		font;

		GameState		state;
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
