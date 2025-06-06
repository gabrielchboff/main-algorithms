export interface Todo {
    id: number;
    title: string;
    description: string;
    isCompleted: boolean;
    createdAt: string;
    completedAt: string | null;
}
