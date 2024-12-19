from rubiks import RubiksCube
import visualize
from vispy import app, scene
from vispy.visuals import transforms
from vispy.color import Color
from numpy import ndarray

def visualization(rubiks: RubiksCube):
    """
    This function creates a visualization to better see the rubiks cube
    """
    # Initialize Canvas
    canvas = scene.SceneCanvas(keys='interactive', show=True)
    view = canvas.central_widget.add_view()
    view.camera = scene.TurntableCamera(distance=6)

    # Create Rubik's Cube
    rubiks_cube = visualize.VisualizeCube(rubiks.state, size=3)
    for cubelet in rubiks_cube.cubelets:
        view.add(cubelet)

    # # Add axes to the scene
    # axes = scene.Axis(
    #     ticks=6,               # Number of ticks on each axis
    #     labels=True,           # Show labels for each axis
    #     axis_width=50,          # Line width for the axes
    #     tick_size=10,          # Size of the tick marks
    #     tick_width=2,          # Width of the tick marks
    #     label_size=20,         # Font size of the axis labels
    #     color='white'          # Axis color
    # )

    # # Position the axes so they are visible in the scene
    # axes.transform = scene.transforms.STTransform(translate=(0, 0, 0))

    # # Add axes to the view
    # view.add(axes)

    # xax = scene.Axis(pos=[[0, 0], [1, 0]], tick_direction=(0, -1), axis_color='r', tick_color='r', text_color='r', font_size=30, parent=view.scene)
    # yax = scene.Axis(pos=[[0, 0], [0, 1]], tick_direction=(-1, 0), axis_color='g', tick_color='g', text_color='g', font_size=30, parent=view.scene)

    # zax = scene.Axis(pos=[[0, 0], [-1, 0]], tick_direction=(0, -1), axis_color='b', tick_color='b', text_color='b', font_size=30, parent=view.scene)
    # zax.transform = scene.transforms.MatrixTransform()  # its acutally an inverted xaxis
    # zax.transform.rotate(90, (0, 1, 0))  # rotate cw around yaxis
    # zax.transform.rotate(-45, (0, 0, 1))  # tick direction towards (-1,-1)

    app.run()

if __name__ == "__main__":
    # initialize rubiks cube
    rubiks = RubiksCube()
    rubiks.U()
    rubiks.R()
    # rubiks.Up()
    # rubiks.U()
    # rubiks.U()

    # print(rubiks.state)

    visualization(rubiks)
