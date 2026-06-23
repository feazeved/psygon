#define SDL_MAIN_USE_CALLBACKS 1 /* use the callbacks instead of main() */
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>
#include <SDL3_image/SDL_image.h>

#include "AppState.hpp"
#include "handleEvents.hpp"
#include "render.hpp"

/* This function runs once at startup. */
SDL_AppResult	SDL_AppInit(void **appstate, int argc, char **argv)
{
	(void)argc; (void)argv;

	AppState*	app = new AppState();

	SDL_SetAppMetadata("Deckbuilder", "0.0", "com.example.mygame");

	if (!SDL_Init(SDL_INIT_VIDEO)) {
		SDL_Log("Couldn't initialize SDL: %s", SDL_GetError());
		delete app;
		return (SDL_APP_FAILURE);
	}

	if (!SDL_CreateWindowAndRenderer("Deckbuilder", 1500, 800, SDL_WINDOW_RESIZABLE, &app->window, &app->renderer)) {
        SDL_Log("Couldn't create window/renderer: %s", SDL_GetError());
        delete app;
        return (SDL_APP_FAILURE);
    }

	SDL_SetRenderLogicalPresentation(app->renderer, 1500, 800, SDL_LOGICAL_PRESENTATION_LETTERBOX);

	app->backgroundTexture = IMG_LoadTexture(app->renderer, "./assets/background.png");
	if (!app->backgroundTexture) {
		SDL_Log("fail texture: %s", SDL_GetError());
		delete app;
        return (SDL_APP_FAILURE);
	}

	*appstate = app;
	return (SDL_APP_CONTINUE);
}

/* This function runs when a new event (mouse input, keypresses, etc) occurs. */
SDL_AppResult	SDL_AppEvent(void *appstate, SDL_Event *event)
{
	AppState*	app = static_cast<AppState*>(appstate);

	if (event->type == SDL_EVENT_QUIT)
		return (SDL_APP_SUCCESS);

	// These will be removed once everything else is working
	if (event->type == SDL_EVENT_KEY_DOWN)
		app->keys[event->key.scancode] = true;
	else if (event->type == SDL_EVENT_KEY_UP)
		app->keys[event->key.scancode] = false;

	switch (app->state)
	{
		case (GameState::STATE_MENU):		handleEvents::handleMenuEvent(*app, *event);		break ;
		//case (GameState::STATE_PLAYING):	handlePlayingEvent(app, event);		break ;
		//case (GameState::STATE_SETTINGS):	handleSettingsEvent(app, event);	break ;
		default: SDL_Log("Not everything implemented yet.\n"); break ;
	}

	return (SDL_APP_CONTINUE);
}

/* This function runs once per frame, and is the heart of the program. */
SDL_AppResult	SDL_AppIterate(void *appstate)
{
	AppState*	app = static_cast<AppState*>(appstate);

	// This will be removed when everything else is implemented
	if (app->keys[SDL_SCANCODE_ESCAPE])
		return (SDL_APP_SUCCESS);

	switch (app->state)
	{
		case (GameState::STATE_MENU):		/*updateMenu(app);*/		render::renderMenu(*app);		break ;
		//case (GameState::STATE_PLAYING):	updatePlaying(app);		renderPlaying(app);		break ;
		//case (GameState::STATE_SETTINGS):	updateSettings(app);	renderSettings(app);	break ;
		//case (GameState::STATE_QUIT):		return (SDL_APP_SUCCESS);
		default: SDL_Log("Not everything implemented yet iterate.\n"); break ;
	}

	return (SDL_APP_CONTINUE);
}

/* This function runs once at shutdown. */
void	SDL_AppQuit(void *appstate, SDL_AppResult result)
{
	(void)result;
	AppState*	app = static_cast<AppState*>(appstate);

	if (app)
	{
		SDL_DestroyTexture(app->backgroundTexture);
		SDL_DestroyRenderer(app->renderer);
		SDL_DestroyWindow(app->window);
		delete app;
	}

	SDL_Quit();
}
