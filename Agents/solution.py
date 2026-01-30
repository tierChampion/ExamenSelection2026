"""
Robot Rescue Challenge - Solution Template

Implement your solution in the `solve` function below.
Your goal is to rescue the person trapped in the building as quickly as possible.

Available Robot methods:
    - robot.move(Direction) -> bool: Move in a direction (FORWARD, BACKWARD, LEFT, RIGHT)
    - robot.sense_fires_around() -> int: Get count of fires in adjacent cells (only cardinal directions, no diagonals) (does not cost time)
    - robot.scan_fires() -> Set[Position]: Get fire positions in cells around the robot (costs 10 seconds)
    - robot.position -> Position: Current robot position
    - robot.is_carrying_person -> bool: Whether robot is carrying someone
    - robot.get_grid_dimensions() -> Tuple[int, int]: Get (width, height)
    - robot.get_exit_position() -> Position: Get exit position
    - robot.get_person_position() -> Position: Get the person's position (known location)

Movement costs:
    - Each move: 1 second
    - Scan: 10 seconds

Rules:
    - There is exactly one person to rescue
    - The person's location is known from the start
    - Stepping on fire destroys the robot (mission fails immediately)
    - Robot starts at the exit position
    - Person is picked up automatically when robot reaches their cell
    - Mission ends automatically when robot returns to exit with the person

Objective: Navigate to the person, pick them up, and return to the exit as quickly as possible!
"""

from robot import Robot, Direction, Position


def solve(robot: Robot) -> None:
    """
    Implement your rescue algorithm here.

    Args:
        robot: The robot instance to control

    Note:
        The mission ends automatically when you return to the exit with the person.
        If the robot steps on fire, the mission fails immediately.
    """
    # TODO: Implement your solution here

    # Example: Get grid info
    width, height = robot.get_grid_dimensions()
    exit_pos = robot.get_exit_position()
    person_pos = robot.get_person_position()

    # Example: Check for nearby fires
    fire_count = robot.sense_fires_around()

    potential_fires = set()
    fires = set()
    safe = set()
    safe.add(person_pos)

    def is_move_safe(dir) -> bool:
        dx, dy = dir.values
        new_pos = Position(robot.position.x + dx, robot.position.y + dy)
        return new_pos in safe

    while not robot.is_carrying_person or robot.position != exit_pos:
        fire_count = robot.sense_fires_around()
        if fire_count == 0:
            safe.add(Position(robot.position.x + 1, robot.position.y))
            safe.add(Position(robot.position.x - 1, robot.position.y))
            safe.add(Position(robot.position.x, robot.position.y + 1))
            safe.add(Position(robot.position.x, robot.position.y - 1))
        else:
            potential_fires.add(Position(robot.position.x + 1, robot.position.y))
            potential_fires.add(Position(robot.position.x - 1, robot.position.y))
            potential_fires.add(Position(robot.position.x, robot.position.y + 1))
            potential_fires.add(Position(robot.position.x, robot.position.y - 1))
        potential_fires = potential_fires.difference(safe)

        if not robot.is_carrying_person:
            person_delta = Position(person_pos.x - robot.position.x, person_pos.y - robot.position.y)
            if person_delta.y < 0:
                robot.move(Direction.FORWARD)
            elif person_delta.y > 0:
                robot.move(Direction.BACKWARD)
            elif person_delta.x < 0:
                robot.move(Direction.LEFT)
            elif person_delta.x > 0:
                robot.move(Direction.RIGHT)
        else:
            exit_delta = Position(exit_pos.x - robot.position.x, exit_pos.y - robot.position.y)
            if exit_delta.y < 0:
                robot.move(Direction.FORWARD)
            elif exit_delta.y > 0:
                robot.move(Direction.BACKWARD)
            elif exit_delta.x < 0:
                robot.move(Direction.LEFT)
            elif exit_delta.x > 0:
                robot.move(Direction.RIGHT)

    # Navigate to person and return to exit - mission ends automatically!
