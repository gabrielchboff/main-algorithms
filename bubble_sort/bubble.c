#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdbool.h>


void array_print(int arr[], int arr_size)
{
    for (int i = 0; i < arr_size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}


void bubble(int arr[], int arr_size){
    for (int i = 0; i < arr_size - 1; i++) {
        for (int j = 0; j < arr_size - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                printf("Array after %d iteration: ", i + 1);
                array_print(arr, arr_size);
                printf("\n");
                sleep(1);
                system("clear");
            }
        }
    }

}


void reverse_bubble(int arr[], int arr_size){
    for (int i = 0; i < arr_size - 1; i++) {
        for (int j = 0; j < arr_size - i - 1; j++) {
            if (arr[j] < arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                printf("Array after %d iteration: ", i + 1);
                array_print(arr, arr_size);
                printf("\n");
                sleep(1);
                system("clear");
            }
        }
    }
}

int main(int argc, char *argv[])
{
    int arr[] = {64, 34, 25, 12, 22, 11, 90};
    int arr_size = sizeof(arr) / sizeof(arr[0]);
    bubble(arr, arr_size);
    printf("Sorted array: \n");
    array_print(arr, arr_size);
    reverse_bubble(arr, arr_size);
    printf("Sorted reverse array: \n");
    array_print(arr, arr_size);

    return EXIT_SUCCESS;
}
