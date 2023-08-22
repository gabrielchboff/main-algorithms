import "fmt"

func main() {
  fmt.Println("100 doors")
  steps := 1 
  doors := [100]bool{}
  while(steps <= 100) {
    for i := 1; i <=100; i += steps {
      doors[i] = !doors[i]      
    }
    steps++;
  }

  fmt.Println(doors)
}
