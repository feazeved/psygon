#include "handleEvents.hpp"

void	handleEvents::handleMenuEvent(App& app, SDL_Event& event)
{
	Menu&	menu = app.mainMenu;

	if (event.type == SDL_EVENT_KEY_DOWN)
	{
		if (event.key.scancode == SDL_SCANCODE_UP)
			menu.moveUp();
		else if (event.key.scancode == SDL_SCANCODE_DOWN)
			menu.moveDown();
		else if (event.key.scancode == SDL_SCANCODE_RETURN)
			app.state = menu.items[menu.curr].targetState;
	}
	else if (event.type == SDL_EVENT_MOUSE_MOTION)
	{
		int	hover = menu.hitTest(event.motion.x, event.motion.y);
		if (hover != -1)
			menu.curr = hover;
	}
	else if (event.type == SDL_EVENT_MOUSE_BUTTON_DOWN)
	{
		int clicked = menu.hitTest(event.button.x, event.button.y);
		if (clicked != -1)
			app.state = menu.items[menu.curr].targetState;
	}

}
