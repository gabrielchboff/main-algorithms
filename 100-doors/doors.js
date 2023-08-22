

const doors_passer = (doors) => {
  let step = 1;
  while (step <= 100) {
    for (let i = 0; i < doors.length; i++) {
      doors[i] = !doors[i];
    }
  }
  return doors
}

(() => {
  let doors = new Array(100).fill(false);
  current_doors = doors_passer(doors);
  console.log(current_doors);
})
