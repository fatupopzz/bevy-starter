# Flappy Piggy 🐷

Un clon del clásico juego Flappy Bird con temática de cerdito, desarrollado en Rust utilizando el motor de juegos Bevy.



## Descripción

Flappy Piggy es una implementación del clásico juego Flappy Bird, donde el jugador controla un pájaro que debe volar entre tuberías sin colisionar con ellas. El juego demuestra la potencia y seguridad de Rust para el desarrollo de videojuegos, aprovechando la arquitectura ECS (Entity Component System) de Bevy.

## Características

- Jugabilidad clásica de Flappy Bird
- Física realista (gravedad, saltos)
- Sistema de puntuación
- Generación procedural de obstáculos
- Detección de colisiones
- Efectos de sonido
- Animaciones fluidas

## Requisitos

- Rust (edición 2021 o superior)
- Cargo (incluido con Rust)

## Instalación

1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/flappy-piggy.git
   cd flappy-piggy
   ```

2. Compila y ejecuta el juego:
   ```bash
   cargo run
   ```

   Nota: La primera compilación puede tardar más debido a la compilación de dependencias.

## Controles

- **Barra espaciadora**: Saltar/Iniciar juego
- **Esc**: Salir del juego

## Estructura del Proyecto

```
flappy-piggy/
├── src/
│   ├── components.rs    # Definición de componentes del juego
│   ├── constants.rs     # Constantes globales
│   ├── main.rs          # Punto de entrada y configuración
│   ├── plugin.rs        # Plugin de configuración de ventana
│   ├── resources.rs     # Recursos del juego (puntuación, estado)
│   ├── setup.rs         # Configuración inicial de entidades
│   ├── systems.rs       # Sistemas del juego (física, colisiones, etc.)
│   └── utils.rs         # Funciones de utilidad
├── assets/
│   ├── texture/         # Texturas del juego
│   └── audio/           # Efectos de sonido
├── Cargo.toml           # Manifiesto y dependencias
└── README.md            # Este archivo
```

## Conceptos Técnicos

Este proyecto demuestra varios conceptos importantes de Rust y Bevy:

- **Sistema ECS** (Entity Component System)
- **Ownership y Borrowing** para gestión segura de memoria
- **Pattern Matching** con enums para el manejo de estados
- **Traits** para comportamientos compartidos
- **Sistemas reactivos** a eventos y estados del juego

## Desarrollo

Este proyecto fue desarrollado como una exploración del desarrollo de juegos en Rust utilizando Bevy. Demuestra cómo Rust puede proporcionar:

1. Alto rendimiento sin sacrificar la seguridad
2. Prevención de errores comunes en tiempo de compilación
3. Gestión de recursos eficiente sin recolector de basura
4. Código conciso pero legible

## Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo LICENSE para más detalles.

## Reconocimientos

- Inspirado en el juego original Flappy Bird de Dong Nguyen
- Desarrollado con el framework [Bevy](https://bevyengine.org/)
- Assets gráficos y de sonido de [fuente]

---

Desarrollado con ❤️, 🦀 (Rust) y 🐷 (Piggy Power!)
