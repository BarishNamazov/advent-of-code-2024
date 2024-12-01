package main

import (
    "bufio"
    "fmt"
    "os"
    "sort"
    "strconv"
    "strings"
)

type Input struct {
    A []int
    B []int
}

func (i *Input) Len() int {
    return len(i.A)
}

func abs(x int) int {
    if x < 0 {
        return -x
    }
    return x
}

func part1(input *Input) int {
    sort.Ints(input.A)
    sort.Ints(input.B)
    sum := 0
    for i := 0; i < input.Len(); i++ {
        sum += abs(input.A[i] - input.B[i])
    }
    return sum
}

func part2(input *Input) int {
    freqB := make(map[int]int)
    for _, b := range input.B {
        freqB[b]++
    }

    sum := 0
    for _, a := range input.A {
        sum += a * freqB[a]
    }
    return sum
}

func main() {
    file, _ := os.Open("input.txt")
    defer file.Close()

    var a, b []int
    scanner := bufio.NewScanner(file)

    for scanner.Scan() {
        line := scanner.Text()
        if line == "" {
            break
        }
        parts := strings.Fields(line)
        if x, err := strconv.Atoi(parts[0]); err == nil {
            a = append(a, x)
        }
        if x, err := strconv.Atoi(parts[1]); err == nil {
            b = append(b, x)
        }
    }

    input := &Input{a, b}

    fmt.Println(part1(input))
    fmt.Println(part2(input))
}
