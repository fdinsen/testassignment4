package com.example.mockito.services;

import com.example.mockito.model.Pokemon;

import java.util.LinkedList;

public class PokemonCenter {
    private LinkedList<Pokemon> pokemons;
    private INurse nurse;
    public PokemonCenter(INurse nurse) {
        this.nurse = nurse;
        pokemons = new LinkedList<>();
    }

    public void accept(Pokemon pokemon) {
        pokemons.add(pokemon);
    }

    public Pokemon collect() {
        if (!pokemons.isEmpty()) {
            return nurse.heal(pokemons.pop());
        }
        return null;
    }
}