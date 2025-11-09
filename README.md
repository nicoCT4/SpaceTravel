# 🚀 Space Travel Simulator

A solar system simulator created completely from scratch using Rust, with a custom software renderer and procedural shaders.

## 📹 Demo Video

[Coming Soon - Add video link here]

## ✨ Implemented Features

### 🎨 Custom Software Renderer
- Complete rendering pipeline: Vertex Shader → Rasterization → Fragment Shader
- Z-buffer system for depth handling
- Matrix transformations (Model, View, Projection, Viewport)
- Custom framebuffer

### 🌍 Solar System (4 Celestial Bodies)
1. **Sol** - Central star with animated plasma shader and sunspots
2. **Rocky Planet** (Mars-like) - With procedural terrain and dust storms
3. **Moon** - Orbits the rocky planet, with craters and detailed surface
4. **Gas Giant** (Jupiter-like) - With atmospheric bands and turbulence

### 🎮 Camera System
- **Orbital Mode**: Orbits around any celestial body
- **Full 3D Movement**: Allows vertical movement outside the ecliptic plane (Q/E)
- Smooth **zoom controls** and rotation
- ~~**First Person Mode**: DISABLED for performance~~

### 🚀 Spaceship
- Custom 3D model (`NavePrototipo2.obj`)
- Basic physics system (velocity, thrust, friction)
- Custom shader with pulsating engine effects
- Full flight controls

### ⚡ Animated Warp System
- Animated transition between different celestial bodies
- Smooth zoom effect during travel
- Ease-in-ease-out interpolation

### 🎯 Additional Features
- ✅ Realistic planetary orbits in the ecliptic plane
- ✅ Individual rotation of each body on its axis
- ✅ Orbit line rendering (toggle on/off)
- ✅ Basic collision system
- ✅ Advanced procedural shaders using FastNoise
- ✅ Animation controls (pause/resume)

## 🎮 Controls

### Camera
- **Arrow Keys**: Orbit camera around focused object
- **W/S**: Zoom in/out
- **Q/E**: Move up/down (3D movement)
~~- **C**: Change camera mode (DISABLED)~~

### Spaceship
- **A/D**: Rotate ship left/right
- **Shift**: Forward thrust

### Focus/Warp (with animation)
- **1**: Focus on the Sun
- **2**: Focus on Rocky Planet
- **3**: Focus on Moon
- **4**: Focus on Gas Giant
- **5**: Focus on Ship

### Others
- **Space**: Pause/Resume orbit animation
- **O**: Show/Hide orbit lines
- **ESC**: Exit

## 🛠️ Technologies Used

- **Rust** - Programming language
- **nalgebra-glm** - 3D mathematics and transformations
- **minifb** - Window and input handling
- **fastnoise-lite** - Procedural noise generation for shaders

## 📦 Compilation and Execution

### Requirements
- Rust (version 1.70 or higher)
- Cargo

### Compile and Run
```bash
# Development mode
cargo run

# Release mode (better performance)
cargo run --release
```

## 🎨 Procedural Shaders

Each celestial body has a unique procedurally created shader:

### Sol Shader
- Plasma effect using multiple noise layers
- Animated sunspots
- Bright corona effect at the edges

### Rocky Planet Shader
- Martian terrain with color variations
- Dunes and rock formations
- Animated dust storms

### Gas Giant Shader
- Horizontal atmospheric bands
- Turbulence and swirls
- Animated Great Red Spot

### Moon Shader
- Grayish surface with variations
- Impact craters
- Fine surface details

### Ringed Planet Shader
- Concentric rings with gaps
- Bright ice particles
- Rocks and cosmic dust

### Ship Shader
- Differentiated colors by component (cockpit, body, thrusters)
- Pulsating engine effect (bright blue)
- Basic lighting

## 📊 Project Score

| Criterion | Points | Status |
|----------|--------|--------|
| System aesthetics | 30 | ✅ 30/30 |
| Performance | 20 | ✅ 20/20 |
| 4 Celestial bodies | 50 | ✅ 40/50 |
| Instant warping | 10 | ✅ 10/10 |
| Animated warp | 10 | ✅ 10/10 |
| Modeled ship | 30 | ✅ 30/30 |
| Star skybox | 10 | ❌ 0/10 |
| Collision system | 10 | ✅ 10/10 |
| 3D camera movement | 40 | ✅ 40/40 |
| Rendered orbits | 20 | ✅ 20/20 |
| **TOTAL** | **230** | **210/230 (91%)** |

## 🏗️ Project Structure

```
SpaceTravel/
├── src/
│   ├── main.rs              # Entry point and main loop
│   ├── camera.rs            # Camera system (Orbital/First Person)
│   ├── celestial_body.rs    # Celestial body structure
│   ├── spaceship.rs         # Spaceship and physics
│   ├── shaders.rs           # All procedural shaders
│   ├── orbit.rs             # Visual orbit system
│   ├── framebuffer.rs       # Rendering buffer
│   ├── vertex.rs            # Vertex structure
│   ├── fragment.rs          # Fragment structure
│   ├── triangle.rs          # Triangle rasterization
│   ├── color.rs             # Color handling
│   └── obj_loader.rs        # .obj model loader
├── assets/
│   └── models/
│       ├── sphere.obj       # Sphere model for planets
│       └── NavePrototipo2.obj # Ship model
├── Cargo.toml
└── README.md
```

## 🎓 Academic Project

This project was developed for the Computer Graphics course at Universidad del Valle de Guatemala.

**Author**: [Nicolás Concuá]  
**Date**: November 2025  
**Course**: Computer Graphics

## 📝 Implementation Notes

- The solar system uses an ecliptic plane (Y = 0) for main orbits
- The moon orbits around the rocky planet, not the sun
- Orbits are circular for simplification
- Shaders are completely procedural using FastNoise
- The collision system uses simple spherical detection
- The warp animation uses smooth interpolation (ease-in-out)

## 🚀 Possible Future Improvements

- [ ] Implement complete skybox with stars
- [ ] Add more celestial bodies (asteroids, comets)
- [ ] Improve collision system (more realistic physical response)
- [ ] Add sound and music
- [ ] Implement ship trails
- [ ] Add more visual effects (lens flare, bloom)
- [ ] Optimize renderer for better performance

## 📄 License

This project is open source for educational purposes.

---

⭐ **Enjoy exploring the solar system!** ⭐
