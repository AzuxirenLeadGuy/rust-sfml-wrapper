# Azuxiren Wrapper for SFML

This game wrapper aims to provide better primitives for game development using SFML. Some of the design has been inspired from the XNA game class, and additional custom helper functions and structs.

Conceptually, the game loop is as shown

```
             ┌───────────┐
             │ Start (A) │
             └───────┬───┘
                     │
                     │
        ┌────────────▼─────────┐
        │Create Game Window (B)│
        └────────────┬─────────┘
                     │
                     │
        ┌────────────▼─────────┐
False   │   while Game Window  │
 ┌──────┤      is open (C)     │◄──────┐
 │      └────────────┬─────────┘       │
 │                   │True             │
 │                   │                 │
 │      ┌────────────▼──────────┐      │
 │      │      Update Game (D)  │      │
 │      └────────────┬──────────┘      │
 │                   │                 │
 │      ┌────────────▼──────────┐      │
 │      │       Draw Game (E)   │      │
 │      └────────────┬──────────┘      │
 │                   │                 │
 │      ┌────────────▼──────────┐      │
 │      │         Delay  (F)    ├──────┘
 │      └───────────────────────┘
 │
 │      ┌───────────────────────┐
 └─────►|     Close screen  (G) │
        └───────────┬───────────┘
                    │
              ┌─────▼───┐
              │ Exit (H)│
              └─────────┘

```

This game loop is realized by the type `CoreSfmlGameEnum<C, S>`.