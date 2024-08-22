
# Things we probably need some version of from Processing/p5.js in Rust, to make porting simple

Reading through Nature of Code, not wanting to work in JavaScript, and wanting to play aroun with Rust more, lead me to this. Initially and probably in general I'm going to lean a heavily on Bevy here as a backend. It's a game engine, so it has a lot of what we need to make a Rust version of Processing already in terms of geometry, rendering, vectors, and and overall environment structure for things to interact. Perhaps one could strip this way down, pull in the bits we need, and make it standalone the way p5.js is, but I don't think that's worth it and I'm certainly not experienced enough with this type of code to make a go of it yet, not after the missile incident.

While the book likely doesn't use *everything* p5.js has to offer, I'm making a complete list of what it contains and how it names them.

## Shape

### 2D Primitives (mostly obvious)

- arc()
- circle()
- ellipse()
- line()
- point()
- quad()
- rect()
- square()
- triangle()

### 3D Models

- loadModel(): loads a model into a geometry object
- model(): draws geometry to canvas

### 3D Primitives

- beginGeometry()
- box()
- buildGeometry()
- cone()
- cylinder()
- ellipsoid()
- endGeometry()
- freeGeometry()
- plane()
- sphere()
- torus()

### Attributes

- ellipseMode()
- noSmooth()
- rectMode()
- smooth()
- strokeCap()
- strokeJoin()
- strokeWeight()

### Curves

- bezier()
- bezierDetail()
- bezierPoint()
- bezierTangent()
- curve()
- curveDetail()
- curvePoint()
- curveTangent()
- curveTightness()

### Vertex

- beginContour()
- beginShape()
- bezierVertex()
- curveVertex()
- endContour()
- endShape()
- normal()
- quadraticVertex()
- vertex()

#### p5.Geometry

- averageNormals()
- averagePoleNormals()
- calculateBoundingBox()
- clearColors()
- computeFaces()
- computeNormals()
- faces()
- flipU(): flip geometry texture's u-coordinates
- flipV(): flip geometry texture's v-coordinates
- normalize()
- saveObj()
- saveStl()
- uvs
- vertexNormals
- vertices

## Color

### Creating & Reading

- alpha()
- blue()
- brightness()
- color()
- green()
- hue()
- lerpColor()
- lightness()
- red()
- saturation()

### Setting

- background()
- beginClip()
- clear()
- clip()
- colorMode()
- endClip()
- erase()
- fill()
- noErase()
- noFill()
- noStroke()
- stroke()

#### p5.Color

- setAlpha()
- setBlue()
- setGreen()
- setRed()
- toString()

## Typography

### Attributes

- textAlign()
- textAscent()
- textDescent()
- textLeading()
- textSize()
- textStyle()
- textWidth()
- textWrap()

### Loading & Displaying

- loadFont()
- text()
- textFont()

#### p5.Font

- font
- textBounds()
- textToPoints()

## Image

- createImage()
- saveCanvas()
- saveFrames()

### Loading & Displaying

- image()
- imageMode()
- loadImage()
- noTint()
- saveGif()
- tint()

### Pixels

- blend()
- copy()
- delay()
- filter()
- get()
- getCurrentFrame()
- height
- loadPixels()
- mask()
- numFrames()
- pause()
- pixelDensity()
- pixels
- play()
- reset()
- resize()
- save()
- set()
- setFrame()
- updatePixels()
- width

## Transform

- applyMatrix()
- resetMatrix()
- rotate()
- rotateX()
- rotateY()
- rotateZ()
- scale()
- shearX()
- shearY()
- translate()

## Environment

- cursor()
- deltaTime
- describe()
- describeElement()
- displayDensity()
- displayHeight
- displayWidth
- focused
- frameCount
- frameRate()
- fullscreen()
- getTargetFrameRate()
- getURL()
- getURLPath()
- gridOutput()
- height
- noCursor()
- pixelDensity()
- print()
- textOutput()
- webglVersion
- width
- windowHeight
- windowResized()
- windowWidth

## 3D

### Camera

- camera()
- createCamera()
- frustum()
- linePerspective()
- ortho()
- perspective()
- setCamera()

### Interaction

- debugMode()
- noDebugMode()
- orbitControl()

### Lights

- ambientLight()
- directionalLight()
- imageLight()
- lightFalloff()
- lights()
- nolights()
- panorama()
- pointLight()
- sepcularColor()
- spotLight()

### Material

#### p5.Camera

#### p5.Shader

- copyToContext()
- setUniform()

## Rendering

#### p5.Framebuffer

#### p5.Graphics

#### p5.Renderer

## Math

### Calculation

### Noise

### Random

### Trigonometry

### Vector

#### p5.Vector

## IO

### Input

### Output

### Table

### Time & Date

#### p5.Table

#### p5.TableRow

#### p5.XML

## Events

### Acceleration

### Keyboard

### Mouse

### Touch

## DOM (probably will not be directly applicable in the same way)

- changed()
- createA()

#### p5.Element

#### p5.File

#### p5.MediaElement

## Data

### Array Functions

### Conversion

### Dictionary

### LocalStorage

### String Functions

#### p5.NumberDict

#### p5.TypedDict

## Structure

## Constants

- AUTO
- DEGREES
- HALF_PI
- HSB
- P2D
- PI
- QUARTER_PI
- RADIANS
- TAU
- TWO_PI
- VERSION
- WEBGL
- WEBGL2
