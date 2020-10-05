import mongodb from "mongodb";

const connect = async () => {
    const url = process.env.DB_URL;
    return (await mongodb.MongoClient.connect(url)).db("schem-uploader");
}

export default connect;