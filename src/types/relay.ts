export type Channel = {
    id: number;
    label: string;
    description: string;
    isOn: boolean;
};

export type LogLevel = "info" | "success" | "warning" | "error";

export type LogEntry = {
    id: number;
    time: string;
    level: LogLevel;
    message: string;
};