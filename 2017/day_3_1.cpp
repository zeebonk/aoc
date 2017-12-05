#include <algorithm>
#include <iostream>
#include <sstream>

using namespace std;

/*
You come across an experimental new kind of memory stored on an infinite two-
dimensional grid.

Each square on the grid is allocated in a spiral pattern starting at a location
marked 1 and then counting up while spiraling outward. For example, the first
few squares are allocated like this:

17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...

While this is very space-efficient (no squares are skipped), requested data must
be carried back to square 1 (the location of the only access port for this
memory system) by programs that can only move up, down, left, or right. They
always take the shortest path: the Manhattan Distance between the location of
the data and square 1.

For example:

- Data from square 1 is carried 0 steps, since it's at the access port.
- Data from square 12 is carried 3 steps, such as: down, left, left.
- Data from square 23 is carried only 2 steps: up twice.
- Data from square 1024 must be carried 31 steps.

How many steps are required to carry the data from the square identified in your
puzzle input all the way to the access port?
*/

int main() {
    int goal = 0;
    cin >> goal;

    // As the bottom right corner always contains the highest value in a circle
    // we first look for the first right corner value that is larger the goal so
    // we only need to search a single circle for our goal position.
    int i = 1;
    int bottom_right = 0;
    while (bottom_right < goal) {
        i += 2;
        bottom_right = i * i;
    }

    // We calculate the values of the corners so that we know both the postions
    // and the values of the corners.
    int bottom_left = bottom_right - (i - 1) * 1;
    int top_left = bottom_right - (i - 1) * 2;
    int top_right = bottom_right - (i - 1) * 3;
    int bottom_right_low = bottom_right - ((i - 1) * 4) + 1;

    // If the goal is greater or equal than a specific corner, we know that our
    // goal lies in a certain direction of that corner. To find the goals
    // position we just take the difference of the goal and the corner value and
    // add that to the right axis of the corner position:
    int x = 0;
    int y = 0;
    if (goal >= bottom_left) {
        x = -(i / 2);
        y = (i / 2);
        // The goal is at the right of the corner
        x += goal - bottom_left;
    }
    else if (goal >= top_left) {
        x = -(i / 2);
        y = -(i / 2);
        // The goal is below the corner
        y += goal - top_left;

    }
    else if (goal >= top_right) {
        x = (i / 2);
        y = -(i / 2);
        // The goal is left of the corner
        x -= goal - top_right;

    }
    else if (goal >= bottom_right_low) {
        x = (i / 2);
        y = (i / 2) - 1;
        // The goal is above the corner
        y -= goal - bottom_right_low;
    }

    // Now that we have the position of our goal, we calculate the manhatten
    // distance y summing the absolute values of the coordinates:
    cout << abs(x) + abs(y) << endl;
}
