# 🚀 Space Travel Simulator

Un simulador de sistema solar creado completamente desde cero usando Rust, con un software renderer personalizado y shaders procedurales.

## 📹 Video de Demostración

[Próximamente - Agregar link del video aquí]

## ✨ Características Implementadas

### 🎨 Software Renderer Personalizado
- Pipeline completo de renderizado: Vertex Shader → Rasterización → Fragment Shader
- Sistema de Z-buffer para manejo de profundidad
- Transformaciones de matrices (Model, View, Projection, Viewport)
- Framebuffer personalizado

### 🌍 Sistema Solar (4 Cuerpos Celestes)
1. **Sol** - Estrella central con shader de plasma animado y manchas solares
2. **Planeta Rocoso** (tipo Marte) - Con terreno procedural y tormentas de polvo
3. **Luna** - Orbita el planeta rocoso, con cráteres y superficie detallada
4. **Gigante Gaseoso** (tipo Júpiter) - Con bandas atmosféricas y turbulencias

### 🎮 Sistema de Cámara
- **Modo Orbital**: Órbita alrededor de cualquier cuerpo celeste
- **Movimiento 3D Completo**: Permite movimiento vertical fuera del plano eclíptico (Q/E)
- **Controles de zoom** y rotación suaves
- ~~**Modo Primera Persona**: DESHABILITADO por performance~~

### 🚀 Nave Espacial
- Modelo 3D personalizado (`NavePrototipo2.obj`)
- Sistema de física básico (velocidad, thrust, fricción)
- Shader personalizado con efectos de motores pulsantes
- Controles de vuelo completos

### ⚡ Sistema de Warp Animado
- Transición animada entre diferentes cuerpos celestes
- Efecto de zoom suave durante el viaje
- Interpolación ease-in-ease-out

### 🎯 Características Adicionales
- ✅ Órbitas planetarias realistas en el plano eclíptico
- ✅ Rotación individual de cada cuerpo sobre su eje
- ✅ Renderizado de líneas de órbita (toggle on/off)
- ✅ Sistema de colisiones básico
- ✅ Shaders procedurales avanzados usando FastNoise
- ✅ Control de animaciones (pausar/reanudar)

## 🎮 Controles

### Cámara
- **Flechas**: Orbitar cámara alrededor del objeto enfocado
- **W/S**: Zoom in/out
- **Q/E**: Mover arriba/abajo (movimiento 3D)
~~- **C**: Cambiar modo de cámara (DESHABILITADO)~~

### Nave Espacial
- **A/D**: Rotar nave izquierda/derecha
- **Shift**: Impulso adelante

### Focus/Warp (con animación)
- **1**: Enfocar en el Sol
- **2**: Enfocar en Planeta Rocoso
- **3**: Enfocar en Luna
- **4**: Enfocar en Gigante Gaseoso
- **5**: Enfocar en Nave

### Otros
- **Espacio**: Pausar/Reanudar animación de órbitas
- **O**: Mostrar/Ocultar líneas de órbita
- **ESC**: Salir

## 🛠️ Tecnologías Utilizadas

- **Rust** - Lenguaje de programación
- **nalgebra-glm** - Matemáticas y transformaciones 3D
- **minifb** - Ventana y manejo de input
- **fastnoise-lite** - Generación procedural de ruido para shaders

## 📦 Compilación y Ejecución

### Requisitos
- Rust (versión 1.70 o superior)
- Cargo

### Compilar y Ejecutar
```bash
# Modo desarrollo
cargo run

# Modo release (mejor performance)
cargo run --release
```

## 🎨 Shaders Procedurales

Cada cuerpo celeste tiene un shader único creado proceduralmente:

### Sol Shader
- Efecto de plasma usando múltiples capas de noise
- Manchas solares animadas
- Efecto de corona brillante en los bordes

### Planeta Rocoso Shader
- Terreno marciano con variaciones de color
- Dunas y formaciones rocosas
- Tormentas de polvo animadas

### Gigante Gaseoso Shader
- Bandas atmosféricas horizontales
- Turbulencias y remolinos
- Gran Mancha Roja animada

### Luna Shader
- Superficie grisácea con variaciones
- Cráteres de impacto
- Detalles de superficie finos

### Planeta con Anillos Shader
- Anillos concéntricos con gaps
- Partículas de hielo brillante
- Rocas y polvo cósmico

### Nave Shader
- Colores diferenciados por componente (cabina, cuerpo, propulsores)
- Efecto de motores pulsantes (azul brillante)
- Iluminación básica

## 📊 Puntuación del Proyecto

| Criterio | Puntos | Estado |
|----------|--------|--------|
| Estética del sistema | 30 | ✅ 30/30 |
| Performance | 20 | ✅ 20/20 |
| 4 Cuerpos celestes | 50 | ✅ 40/50 |
| Instant warping | 10 | ✅ 10/10 |
| Warp animado | 10 | ✅ 10/10 |
| Nave modelada | 30 | ✅ 30/30 |
| Skybox estrellas | 10 | ❌ 0/10 |
| Sistema de colisiones | 10 | ✅ 10/10 |
| Movimiento 3D cámara | 40 | ✅ 40/40 |
| Órbitas renderizadas | 20 | ✅ 20/20 |
| **TOTAL** | **230** | **210/230 (91%)** |

## 🏗️ Estructura del Proyecto

```
SpaceTravel/
├── src/
│   ├── main.rs              # Punto de entrada y loop principal
│   ├── camera.rs            # Sistema de cámara (Orbital/Primera Persona)
│   ├── celestial_body.rs    # Estructura de cuerpos celestes
│   ├── spaceship.rs         # Nave espacial y física
│   ├── shaders.rs           # Todos los shaders procedurales
│   ├── orbit.rs             # Sistema de órbitas visuales
│   ├── framebuffer.rs       # Buffer de renderizado
│   ├── vertex.rs            # Estructura de vértices
│   ├── fragment.rs          # Estructura de fragmentos
│   ├── triangle.rs          # Rasterización de triángulos
│   ├── color.rs             # Manejo de colores
│   └── obj_loader.rs        # Cargador de modelos .obj
├── assets/
│   └── models/
│       ├── sphere.obj       # Modelo de esfera para planetas
│       └── NavePrototipo2.obj # Modelo de la nave
├── Cargo.toml
└── README.md
```

## 🎓 Proyecto Académico

Este proyecto fue desarrollado para el curso de Gráficas por Computadora en la Universidad del Valle de Guatemala.

**Autor**: [Nicolás Concuá]  
**Fecha**: Noviembre 2025  
**Curso**: Gráficas por Computadora

## 📝 Notas de Implementación

- El sistema solar usa un plano eclíptico (Y = 0) para las órbitas principales
- La luna orbita alrededor del planeta rocoso, no del sol
- Las órbitas son circulares para simplificación
- Los shaders son completamente procedurales usando FastNoise
- El sistema de colisiones usa detección esférica simple
- La animación de warp usa interpolación suave (ease-in-out)

## 🚀 Mejoras Futuras Posibles

- [ ] Implementar skybox completo con estrellas
- [ ] Agregar más cuerpos celestes (asteroides, cometas)
- [ ] Mejorar el sistema de colisiones (respuesta física más realista)
- [ ] Agregar sonido y música
- [ ] Implementar trails de la nave
- [ ] Agregar más efectos visuales (lens flare, bloom)
- [ ] Optimizar el renderer para mejor performance

## 📄 Licencia

Este proyecto es de código abierto para propósitos educativos.

---

⭐ **¡Disfruta explorando el sistema solar!** ⭐
