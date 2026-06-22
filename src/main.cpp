#define SDL_MAIN_USE_CALLBACKS 1 /* use the callbacks instead of main() */
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>

#include "AppState.hpp"

/* This function runs once at startup. */
SDL_AppResult	SDL_AppInit(void **appstate, int argc, char **argv)
{
	(void)argc; (void)argv;

	AppState*	state = new AppState();

	SDL_SetAppMetadata("Deckbuilder", "0.0", "com.example.mygame");

	if (!SDL_Init(SDL_INIT_VIDEO)) {
		SDL_Log("Couldn't initialize SDL: %s", SDL_GetError());
		return (SDL_APP_FAILURE);
	}

	if (!SDL_CreateWindowAndRenderer("Deckbuilder", 840, 720, SDL_WINDOW_RESIZABLE, &state->window, &state->renderer)) {
        SDL_Log("Couldn't create window/renderer: %s", SDL_GetError());
        return (SDL_APP_FAILURE);
    }

	SDL_SetRenderLogicalPresentation(state->renderer, 640, 480, SDL_LOGICAL_PRESENTATION_LETTERBOX);

	*appstate = state;
	return (SDL_APP_CONTINUE);
}

/* This function runs when a new event (mouse input, keypresses, etc) occurs. */
SDL_AppResult	SDL_AppEvent(void *appstate, SDL_Event *event)
{
	AppState*	state = static_cast<AppState*>(appstate);

	switch (event->type)
	{
		case (SDL_EVENT_QUIT):
			return (SDL_APP_SUCCESS);

		case (SDL_EVENT_KEY_DOWN):
			state->keys[event->key.scancode] = true;
			break ;

		case (SDL_EVENT_KEY_UP):
			state->keys[event->key.scancode] = false;
			break ;
	}

	return (SDL_APP_CONTINUE);
}

/* This function runs once per frame, and is the heart of the program. */
SDL_AppResult	SDL_AppIterate(void *appstate)
{
	(void)appstate;
	AppState*	state = static_cast<AppState*>(appstate);

	if (state->keys[SDL_SCANCODE_ESCAPE])
	{
		return (SDL_APP_SUCCESS);
	}



	SDL_SetRenderDrawColorFloat(state->renderer, 255, 255, 255, SDL_ALPHA_TRANSPARENT_FLOAT); /* new color, full alpha. */

	/* clear the window to the draw color. */
	SDL_RenderClear(state->renderer);

	/* put the newly-cleared rendering on the screen. */
	SDL_RenderPresent(state->renderer);

	return (SDL_APP_CONTINUE);
}

/* This function runs once at shutdown. */
void	SDL_AppQuit(void *appstate, SDL_AppResult result)
{
	(void)result;
	AppState*	state = static_cast<AppState*>(appstate);

	if (state)
	{
		SDL_DestroyRenderer(state->renderer);
		SDL_DestroyWindow(state->window);
		delete state;
	}

	SDL_Quit();
}
