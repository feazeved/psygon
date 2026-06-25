#define SDL_MAIN_USE_CALLBACKS 1 /* use the callbacks instead of main() */
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>
#include <SDL3_image/SDL_image.h>

#include "App.hpp"
#include "handleEvents.hpp"
#include "render.hpp"

/* This function runs once at startup. */
SDL_AppResult	SDL_AppInit(void **appstate, int argc, char **argv)
{
	(void)argc; (void)argv;
	SDL_SetAppMetadata("Deckbuilder", "0.0", "com.example.mygame");

	App*	App = nullptr;

	try {
		App = new class App();

	} catch (const std::exception& e) {
		SDL_Log("Error: %s: %s\nExiting...\n", e.what(), SDL_GetError());

		delete App;
		return (SDL_APP_FAILURE);
	}

	*appstate = App;
	return (SDL_APP_CONTINUE);
}

/* This function runs when a new event (mouse input, keypresses, etc) occurs. */
SDL_AppResult	SDL_AppEvent(void *appstate, SDL_Event *event)
{
	App*	App = static_cast<class App*>(appstate);

	if (event->type == SDL_EVENT_QUIT)
		return (SDL_APP_SUCCESS);

	switch (App->state)
	{
		case (GameState::STATE_MENU):		handleEvents::handleMenuEvent(*App, *event);		break ;
		//case (GameState::STATE_PLAYING):	handlePlayingEvent(app, event);		break ;
		//case (GameState::STATE_SETTINGS):	handleSettingsEvent(app, event);	break ;
		default: SDL_Log("Not everything implemented yet.\n"); break ;
	}

	return (SDL_APP_CONTINUE);
}

/* This function runs once per frame, and is the heart of the program. */
SDL_AppResult	SDL_AppIterate(void *appstate)
{
	App*	App = static_cast<class App*>(appstate);

	// This will be removed when everything else is implemented
	if (App->keys[SDL_SCANCODE_ESCAPE])
		return (SDL_APP_SUCCESS);

	switch (App->state)
	{
		case (GameState::STATE_MENU):		/*updateMenu(app);*/		render::renderMenu(*App);		break ;
		//case (GameState::STATE_PLAYING):	updatePlaying(app);		renderPlaying(app);		break ;
		//case (GameState::STATE_SETTINGS):	updateSettings(app);	renderSettings(app);	break ;
		//case (GameState::STATE_QUIT):		return (SDL_APP_SUCCESS);
		default: SDL_Log("Not everything implemented yet iterate.\n"); break ;
	}

	SDL_RenderPresent(App->renderer);

	return (SDL_APP_CONTINUE);
}

/* This function runs once at shutdown. */
void	SDL_AppQuit(void *appstate, SDL_AppResult result)
{
	(void)result;
	App*	App = static_cast<class App*>(appstate);

	delete App;

	SDL_Quit();
}
