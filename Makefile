CC = c++

LD_FLAGS	= -L/usr/local/lib -Wl,-rpath,/usr/local/lib -lSDL3
CFLAGS		= -Wall -Wextra -Werror -I/usr/local/include -std=c++23

NAME		= deckbuilder

SRC_DIR		= src
OBJ_DIR		= obj

SRC			= main.cpp AppState.cpp
OBJ			= $(addprefix $(OBJ_DIR)/, $(SRC:.cpp=.o))

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
