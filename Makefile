CC          = c++

CFLAGS      = -Wall -Wextra -Werror -std=c++23 $(shell pkg-config --cflags sdl3 sdl3-image sdl3-ttf)
LD_FLAGS    = $(shell pkg-config --libs sdl3 sdl3-image sdl3-ttf)

NAME        = psygon

SRC_DIR     = src
OBJ_DIR     = obj

SRC         = main.cpp App.cpp handleEvents.cpp render.cpp Menu.cpp
OBJ         = $(addprefix $(OBJ_DIR)/, $(SRC:.cpp=.o))

all: $(NAME)

$(NAME): $(OBJ)
	$(CC) $(OBJ) -o $(NAME) $(LD_FLAGS)

$(OBJ_DIR):
	mkdir -p $(OBJ_DIR)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp | $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

run:
	make re
	clear
	./$(NAME)

clean:
	rm -rf $(OBJ_DIR)

fclean: clean
	rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re run
