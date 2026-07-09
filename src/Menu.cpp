#include <cstddef>
#include "Menu.hpp"

// ----- Class implementation -----
Menu::Menu() : curr(0) {}

Menu::~Menu() {}


// ----- Public Methods -----
void	Menu::moveUp()
{
	curr = (curr - 1 + items.size()) % items.size();
}

void	Menu::moveDown()
{
	curr = (curr + 1) % items.size();
}

void	Menu::addItem(const std::string& label, const SDL_FRect& r, GameState target)
{
	items.push_back({label, r, target});
}

int		Menu::hitTest(float mx, float my) const
{
	for (std::size_t i = 0; i < items.size(); i++)
	{
		const SDL_FRect r = items[i].rect;
		if (mx >= r.x && mx <= r.x + r.w && my >= r.y && my <= r.y + r.h)
			return (static_cast<int>(i));
	}
	return (-1);
}
