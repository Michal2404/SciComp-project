# import stuff here
import numpy as np
from numpy import ndarray
from typing import Tuple

class RubiksCube:
    def __init__(self, size = 3, face_num = 6, location_num = 9):
        self.size = size
        self.face_num = face_num
        self.location_num = location_num
        self.state = np.repeat(np.arange(1, face_num+1).reshape(face_num, 1), location_num, axis=1)
        # self.state[1, :] = np.array([1,2,3,4,5,6,1,2,3])
        # self.state[5, :] = np.array([1,2,3,4,5,6,1,2,3])

    def move(self, move: str):
        """
        This function performs the next movement of the rubiks cube
        """
        if move == "U":
            self.U()
        elif move == "Up":
            self.Up()
        elif move == "R":
            self.R()
        elif move == "Rp":
            self.Rp()
        else:
            raise RuntimeError("Not recognized movement specified")


    def permutation_matrices(self, rotation) -> Tuple[ndarray, ndarray]:
        """
        This fucntion returns the permutation matrices for clockwise and counterclockwise directions
        """
        # for clockwise rotation, we specify the following
        if rotation == "clockwise":
            P1 = np.array([
                [0, 0, 0, 1],
                [0, 0, 1, 0],
                [1, 0, 0, 0],
                [0, 1, 0, 0]
            ])
            P2 = np.array([
                [0, 0, 1, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 1],
                [0, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 0],
                [1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 1, 0, 0]
            ])

        else:
            P1 = np.array([
                [0, 0, 1, 0],
                [0, 0, 0, 1],
                [0, 1, 0, 0],
                [1, 0, 0, 0]
            ])
            P2 = np.array([
                [0, 0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 1, 0, 0, 0, 0, 0, 0]
            ])

        return P1, P2

    def Up(self):
        """
        Perform the U (Up) permutation on the cube state.
        This rotates the top face counterclockwise and updates the affected sides.
        """
        n = self.state.shape[0]

        # Permutation matrices
        P1, P2 = self.permutation_matrices("counterclockwise")
        
        P_sides = np.eye(n)
        P_sides[2:, 2:] = P1

        # Apply side permutation
        temp = P_sides @ self.state

        # Apply affected portion to the statee
        self.state[2:, 0:3] = temp[2:, 0:3]

        # Rotate the top face
        self.state[1, :] = self.state[1, :] @ P2


    def U(self):
        """
        Perform the U (Up) permutation on the cube state.
        This rotates the top face clockwise and updates the affected sides.
        """
        n = self.state.shape[0]

        # Permutation matrices
        P1, P2 = self.permutation_matrices("clockwise")
        P_sides = np.eye(n)
        P_sides[2:, 2:] = P1

        # Apply side permutation
        temp = P_sides @ self.state

        # Apply affected portion to the state
        self.state[2:, 0:3] = temp[2:, 0:3]

        # Rotate the top face
        self.state[1, :] = self.state[1, :] @ P2


    def R(self):
        """
        Perform the R (Right) permutation on the cube state.
        This rotates the right face clockwise and updates the affected sides.
        """
        n = self.state.shape[0]

        # Permutation matrices
        P1, P2 = self.permutation_matrices("clockwise")
        P_sides = np.eye(n)
        P_sides[:4, :4] = P1

        # Apply side permutation
        temp = P_sides @ self.state

        # Apply affected portion to the statee
        self.state[0:4, 2::3] = temp[0:4, 2::3]
        # switch positioning for y=1
        P3 = np.array([
            [0, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0]
        ])
        self.state[3, :] = self.state[3, :] @ P3

        # Rotate the right face
        self.state[5, :] = self.state[5, :] @ P2

    def Rp(self):
        """
        Perform the Rp (Right prime) permutation on the cube state.
        This rotates the right face counterclockwise and updates the affected sides.
        """
        n = self.state.shape[0]

        # Permutation matrices
        P1, P2 = self.permutation_matrices("counterclockwise")
        P_sides = np.eye(n)
        P_sides[:4, :4] = P1

        # Apply side permutation
        temp = P_sides @ self.state

        # Apply affected portion to the statee
        self.state[0:4, 2::3] = temp[0:4, 2::3]
        # switch positioning for y=1
        P3 = np.array([
            [0, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0]
        ])
        self.state[3, :] = self.state[3, :] @ P3

        # Rotate the right face
        self.state[5, :] = self.state[5, :] @ P2



if __name__ == "__main__":
    rubiks = RubiksCube()
    # rubiks.Up()

    print(rubiks.state)
