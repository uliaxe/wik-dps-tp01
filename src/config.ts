import dotenv from "dotenv";

dotenv.config();

export const PORT: number = parseInt(process.env.PING_LISTEN_PORT || "8080", 10);
