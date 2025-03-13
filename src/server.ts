import express, { Request, Response } from "express";
import { PORT } from "./config";

const app = express();

// Route /ping qui retourne les headers
app.get("/ping", (req: Request, res: Response) => {
    res.json(req.headers);
});

// Gestion des routes inexistantes (404)
app.use((req: Request, res: Response) => {
    res.status(404).send();
});

// Démarrer le serveur
app.listen(PORT, () => {
    console.log(`🚀 Serveur en écoute sur le port ${PORT}`);
});
