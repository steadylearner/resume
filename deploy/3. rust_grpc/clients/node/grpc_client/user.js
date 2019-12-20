const grpc = require('grpc');
const protoLoader = require('@grpc/proto-loader');

const USER_PATH = "./proto/user.proto";
const options = require("../options");

const userDefinition = protoLoader.loadSync(
    USER_PATH,
    options
);
const userproto = grpc.loadPackageDefinition(userDefinition);

const UserService = userproto.UserService;

const user_grpc = new UserService('0.0.0.0:50051', grpc.credentials.createInsecure());

module.exports = user_grpc;