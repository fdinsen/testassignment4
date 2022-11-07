package com.example.mockito.model;

public class Pokemon {
    private String name;
    private int hp;

    public Pokemon(String name, int hp) {
        this.name = name;
        this.hp = hp;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public int getHp() {
        return hp;
    }

    public void setHp(int hp) {
        this.hp = hp;
    }

    public Pokemon copy() {
        return new Pokemon(String.copyValueOf(name.toCharArray()), hp);
    }

    @Override
    public boolean equals(Object o) {
        if (o == this) {
            return true;
        }

        /* Check if o is an instance of Complex or not
          "null instanceof [type]" also returns false */
        if (!(o instanceof Pokemon)) {
            return false;
        }

        // typecast o to Complex so that we can compare data members
        Pokemon c = (Pokemon) o;

        // Compare the data members and return accordingly
        return name == c.name && hp == c.hp;
    }
}
