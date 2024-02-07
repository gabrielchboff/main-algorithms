#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdbool.h>


void insertationSort(int arr[], int arr_size) {
    for (int i = 1; i < arr_size; i++) {
        int key = arr[i];
        int j = i - 1;
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j--;

        }
        arr[j + 1] = key;
    }
}

void insertationSortReverse(int arr[], int arr_size) {
for (int i = 1; i < arr_size; i++) {
        int key = arr[i];
        int j = i - 1;
        while (j >= 0 && arr[j] < key) {
            arr[j + 1] = arr[j];
            j--;

        }
        arr[j + 1] = key;
    }
}


int main(int argc, char *argv[])
{
    int arr[] = {64, 34, 25, 0, 12, 22, 11, 90};
    int arr_size = sizeof(arr) / sizeof(arr[0]);
    insertationSort(arr, arr_size);
    printf("Sorted array: \n");
    for (int i = 0; i < arr_size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    insertationSortReverse(arr, arr_size);
    printf("Sorted reverse array: \n");
    for (int i = 0; i < arr_size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    return EXIT_SUCCESS;
}
