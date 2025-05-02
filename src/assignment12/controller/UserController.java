package assignment12.controller;

import assignment12.UserService;

public class UserController {

    private final UserService userService = new UserService();

    public String createUser(String name) {
        return userService.createUser(name);
    }
}

