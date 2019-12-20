// https://github.com/prisma/graphql-request

// express-promise-router
const express = require('express')
const router = express.Router()

const { user_grpc } = require("../grpc_client");

router.get("/:id", (req, res) => {
    const { id } = req.params;
    console.log(id);

    user_grpc.GetUser({ id }, (error, response) => {
        if (!error) {
            return res.json({ response, success: true });
        }
        else {
            return res.json({ error, success: false });
        }
    })
});

router.get("/", (_req, res) => {
    user_grpc.ListUsers({}, (error, response) => {
        if (!error) {
            return res.json({ response, success: true });
        }
        else {
            return res.json({ error, success: false });
        }
    })
});

module.exports = router;
