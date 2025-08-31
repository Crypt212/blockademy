
import { HttpAgent, Actor } from "@dfinity/agent";
import { idlFactory } from "../../../declarations/backend/backend.did.js";

const canisterId = "uxrrr-q7777-77774-qaaaq-cai";
const agent = new HttpAgent({ host: "http://127.0.0.1:4943" });
export const backend = Actor.createActor(idlFactory, { agent, canisterId });
