package com.example.mockito;

import com.example.mockito.services.*;
import com.example.mockito.model.*;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;
import org.mockito.internal.verification.VerificationModeFactory;
import org.springframework.boot.test.context.SpringBootTest;

import java.util.function.Predicate;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.mockito.Mockito.*;

@SpringBootTest
class PokemonCenterMockitoTest {

    @Mock
    INurse nurse;

    @BeforeEach
    public void setup() {
        MockitoAnnotations.openMocks(this);
    }
	@Test
	void testPokemonCenter() {
        var pkmnCenter = new PokemonCenter(nurse);

        var input = new Pokemon("Rowlet", 10);
        var expected = new Pokemon("Rowlet", 100);
        when(nurse.heal(input)).thenReturn(expected);

        pkmnCenter.accept(input);
        var actual = pkmnCenter.collect();

        verify(nurse).heal(input);
        assertEquals(expected, actual);
	}

    @Test
    void testPokemonCenterNone() {
        var pkmnCenter = new PokemonCenter(nurse);

        Pokemon input = null;
        Pokemon expected = null;
        when(nurse.heal(input)).thenReturn(expected);

        var actual = pkmnCenter.collect();

        verify(nurse, never()).heal(input);
        assertEquals(expected, actual);
    }

    @Test
    void testPokemonCenterTimes() {
        var pkmnCenter = new PokemonCenter(nurse);

        var input = new Pokemon("Rowlet", 10);
        var expected = new Pokemon("Rowlet", 100);
        when(nurse.heal(input)).thenReturn(expected);

        pkmnCenter.accept(input);
        pkmnCenter.accept(input);
        var actual = pkmnCenter.collect();
        var actual2 = pkmnCenter.collect();

        verify(nurse, VerificationModeFactory.times(2)).heal(input);
        assertEquals(expected, actual);
    }

    @Test
    void testPokemonCenterPredicate() {
        var pkmnCenter = new PokemonCenter(nurse);

        var input = new Pokemon("Rowlet", 10);
        var expected = new Pokemon("Rowlet", 100);
        when(nurse.heal(argThat(
                (Pokemon pkmn) -> pkmn.getName().equals(input.getName())
                        && pkmn.getHp() == input.getHp())
        )).thenReturn(expected);

        pkmnCenter.accept(input.copy());
        var actual = pkmnCenter.collect();

        verify(nurse).heal(argThat(
                (Pokemon pkmn) -> pkmn.getName().equals(input.getName())
                                && pkmn.getHp() == input.getHp())
        );
        assertEquals(expected, actual);
    }

}
