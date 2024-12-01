from dataclasses import dataclass
from collections import Counter


@dataclass
class Input:
    a: list[int]
    b: list[int]

    def __len__(self):
        return len(self.a)


def part1(input: Input) -> int:
    return sum(abs(x - y) for x, y in zip(sorted(input.a), sorted(input.b)))


def part2(input: Input) -> int:
    freq_b = Counter(input.b)
    return sum(x * freq_b.get(x, 0) for x in input.a)


if __name__ == "__main__":
    with open("input.txt", "r") as file:
        data = filter(bool, file.read().splitlines())
        a, b = map(list, zip(*(map(int, line.split()) for line in data)))
        input = Input(a, b)

    print(part1(input))
    print(part2(input))
