#include "App.hpp"

#include <SDL3_image/SDL_image.h>
#include <SDL3/SDL_init.h>
#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>

const int	App::winWidth = 1920;
const int	App::winHeight = 1080;

App::App() :
	window(nullptr), renderer(nullptr), state(GameState::STATE_MENU), backgroundTexture(nullptr), keys(nullptr)
{
	if (!SDL_Init(SDL_INIT_VIDEO))
		throw App::InitException("SDL_Init");

	if (!SDL_CreateWindowAndRenderer("Deckbuilder", App::winWidth, App::winHeight, SDL_WINDOW_RESIZABLE, &window, &renderer))
		throw App::InitException("SDL_CreateWindowAndRenderer");

	if (!SDL_SetRenderLogicalPresentation(renderer, App::winWidth, App::winHeight, SDL_LOGICAL_PRESENTATION_STRETCH))
		throw App::InitException("SDL_SetRenderLogicalPresentation");

	backgroundTexture = IMG_LoadTexture(renderer, "./assets/background.png");
	if (!backgroundTexture)
		throw App::InitException("IMG_Load_Texture (background)");

	float aspect = static_cast<float>(App::winWidth) / static_cast<float>(App::winHeight);
	if (!SDL_SetWindowAspectRatio(window, aspect, aspect))
		throw App::InitException("SDL_SetWindowAspectRatio");

	keys = SDL_GetKeyboardState(NULL);
}

App::~App()
{
	SDL_DestroyTexture(backgroundTexture);
	SDL_DestroyRenderer(renderer);
	SDL_DestroyWindow(window);
}







App::InitException::InitException(const std::string& where) : message("failure initializing " + where) { }

const char*	App::InitException::what() const noexcept { return (message.c_str()); }
