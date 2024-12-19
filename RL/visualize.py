import numpy as np
from vispy import app, scene
from vispy.visuals import transforms
from vispy.color import Color
from numpy import ndarray

# Define color mapping
color_map = {
    1: 'white',   # Bottom (D)
    2: 'yellow',  # Top (U)
    3: 'red',     # Front (F)
    4: 'orange',  # Back (B)
    5: 'blue',    # Left (L)
    6: 'green'    # Right (R)
}

# Rubik's Cube Class
class VisualizeCube:
    def __init__(self, state: ndarray, size=3):
        self.size = size
        self.cubelets = []

        self.state = state
        self.create_cube()
        

    def create_cube(self):
        """Create the 3D Rubik's Cube."""

        # get the current state of the rubiks cube
        offsets = [-1, 0, 1]  # Positions for cubelet centers in X, Y, and Z
        for x in offsets:
            for y in offsets:
                for z in offsets:
                    # Determine colors for each face
                    face_colors = self.get_face_colors(x, y, z)
                    face_colors = np.array([color for color in face_colors for _ in range(2)])
                    # Create a cubelet (3D Box) with face colors
                    cubelet = scene.visuals.Box(
                        width=0.9, height=0.9, depth=0.9,
                        face_colors=face_colors, edge_color='black'
                    )
                    
                    # Position the cubelet
                    cubelet.transform = scene.transforms.STTransform(translate=(x, y, z))
                    self.cubelets.append(cubelet)

    def get_face_colors(self, x, y, z):
        """Get colors for each face of a cubelet based on its position."""
        colors = np.zeros((6, 4))  # 6 faces, 4 vertices per face, RGBA color

        # specify the local coordinate
        a, b = self.local_coordinate()
        local_position = {(int(a[i]), int(b[i])): i for i in range(len(a))}

        # loop for each coordinate
        coordinates = np.array([x, y, z])
        for index, cut in enumerate(coordinates):
            # we continue if cut == 0
            if cut != 0:
                # print(f"x={x}, y={y}, z={z}")
                # print(f"index={index}, cut={cut}")
                # print(np.delete(coordinates, index))
                # print("")
                # we transform the global coordinate system to local
                val = self.transform_global_local(index, cut, np.delete(coordinates, index))
                # get the index for local_position
                j = local_position[(val[0], val[1])]

                # i will depend on the cut 
                # bottom
                if cut == -1 and index == 2: #z=-1
                    colors[0] = Color(color_map[self.state[0, j]]).rgba
                # top
                if cut == 1 and index == 2: #z=1
                    colors[1] = Color(color_map[self.state[1, j]]).rgba
                # front
                if cut == -1 and index == 1: #y=-1
                    colors[2] = Color(color_map[self.state[2, j]]).rgba
                # back
                if cut == 1 and index == 1: #y=1
                    colors[3] = Color(color_map[self.state[3, j]]).rgba
                # left
                if cut == -1 and index == 0: #x=-1
                    colors[4] = Color(color_map[self.state[4, j]]).rgba
                # right
                if cut == 1 and index == 0: #x=1
                    colors[5] = Color(color_map[self.state[5, j]]).rgba

        return colors

    
    def local_coordinate(self):
        """
        This function outputs the local coordinate system given a and b
        """
        a = np.array([-1, 0 ,1])
        b = np.array([1, 0 ,-1])
        A, B = np.meshgrid(a, b)

        return A.flatten(), B.flatten()

    def transform_global_local(self, index, cut, array):
        # for x, we need to change the coordinate, for a
        if index == 0 and cut == -1:
            return np.array([[-1, 0], [0, 1]]) @ array
        # for z, we need to change the coordinate, for b
        elif index == 2 and cut == -1:
            return np.array([[1, 0], [0, -1]]) @ array
        # for y, we need to change the coordinate, for a
        elif index == 1 and cut == 1:
            return np.array([[-1, 0], [0, 1]]) @ array
        # otherwise, we don't need to change anything
        else:
            return array


