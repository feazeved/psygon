#pragma once

#include <string>
#include <vector>
#include <SDL3/SDL_rect.h>

#include "GameState.hpp"

struct	MenuItem
{
	std::string	label;
	SDL_FRect	rect;
	GameState	targetState;
};

class Menu {
public:
	std::vector<MenuItem>	items;
	int						curr;

	Menu();
	Menu(const Menu& other) = default;
	Menu&	operator=(const Menu& other) = default;
	~Menu();

	void	moveUp();
	void	moveDown();

	void	addItem(const std::string& label, const SDL_FRect& r, GameState target);

	int		hitTest(float mx, float my) const;
};
