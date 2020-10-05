import {NextApiRequest, NextApiResponse} from "next";
import connect from "../../../lib/db";
import {Db} from "mongodb";
import filterRequest from "../../../lib/FilterRequest";
import RequestType from "../../../lib/RequestType";

interface LoginBody {
    email: String,
    password: String
}

export default async (req: NextApiRequest, res: NextApiResponse) => {
    if (!filterRequest(req, res, RequestType.POST)) return;
    const db: Db = await connect();
    const {email, password}: LoginBody = req.body;
    if (!email || !password) {
        res.status(400).end();
        return;
    }
    res.status(200).json({email, password, users: db.collection("users").find({email}).toArray()})
}
