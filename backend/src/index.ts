import express, { Request, Response } from "express";

const app = express();
app.use(express.json());

// Mock Database
const profiles: any[] = [];
const connections: any[] = [];
const stakes: any[] = [];

app.post("/api/profiles", (req: Request, res: Response) => {
  const { name, dateOfBirth, gender, graduatedFrom, currentlyWorking } =
    req.body;

  const newProfile = {
    id: profiles.length + 1,
    name,
    dateOfBirth,
    gender,
    graduatedFrom,
    currentlyWorking,
    verifiedFields: [], // Start with no verified fields
  };

  profiles.push(newProfile);
  res
    .status(201)
    .json({ id: newProfile.id, message: "Profile created successfully" });
});

app.get("/api/profiles/:userId", (req: Request, res: Response) => {
  const profile = profiles.find((p) => p.id === parseInt(req.params.userId));
  if (!profile) return res.status(404).json({ message: "Profile not found" });

  res.json(profile);
});

app.put("/api/profiles/:userId", (req: Request, res: Response) => {
  const profile = profiles.find((p) => p.id === parseInt(req.params.userId));
  if (!profile) return res.status(404).json({ message: "Profile not found" });

  Object.assign(profile, req.body);
  res.json({ message: "Profile updated successfully" });
});

app.post("/api/profiles/:userId/verify", (req: Request, res: Response) => {
  const { fieldsToVerify, provider } = req.body;
  const profile = profiles.find((p) => p.id === parseInt(req.params.userId));
  if (!profile) return res.status(404).json({ message: "Profile not found" });

  // Mock a verification link
  const verificationLink = `https://reclaim.protocol/verify?id=${profile.id}`;
  res.json({
    verificationLink,
    message: "Verification initiated successfully",
  });
});

app.get(
  "/api/profiles/:userId/verify/status",
  (req: Request, res: Response) => {
    const profile = profiles.find((p) => p.id === parseInt(req.params.userId));
    if (!profile) return res.status(404).json({ message: "Profile not found" });

    // Simulate a verification result
    profile.verifiedFields = ["name", "graduatedFrom"];
    res.json({
      verifiedFields: profile.verifiedFields,
      message: "Verification completed",
    });
  }
);

app.post("/api/connections/stake", (req: Request, res: Response) => {
  const { userId, stakeAmount } = req.body;
  stakes.push({ userId, stakeAmount });
  res.json({ message: "Stake successfully made" });
});

app.post("/api/connections", (req: Request, res: Response) => {
  const { user1Id, user2Id } = req.body;
  const stake1 = stakes.find(
    (s) => s.userId === user1Id && s.stakeAmount === 0.2
  );
  const stake2 = stakes.find(
    (s) => s.userId === user2Id && s.stakeAmount === 0.2
  );

  if (!stake1 || !stake2)
    return res
      .status(400)
      .json({ message: "Both users must stake 0.2 SOL to connect" });

  const newConnection = {
    id: connections.length + 1,
    user1Id,
    user2Id,
    stakeAmountUser1: stake1.stakeAmount,
    stakeAmountUser2: stake2.stakeAmount,
    retainHistory: true, // Default setting
  };

  connections.push(newConnection);
  res.json({
    connectionId: newConnection.id,
    message: "Connection established successfully",
  });
});

app.post(
  "/api/connections/:connectionId/withdraw-stake",
  (req: Request, res: Response) => {
    const connectionId = parseInt(req.params.connectionId);
    const connectionIndex = connections.findIndex((c) => c.id === connectionId);

    if (connectionIndex === -1)
      return res.status(404).json({ message: "Connection not found" });

    connections.splice(connectionIndex, 1); // Remove connection
    res.json({ message: "Stake withdrawn, connection removed" });
  }
);

app.put(
  "/api/connections/:connectionId/chat-settings",
  (req: Request, res: Response) => {
    const connectionId = parseInt(req.params.connectionId);
    const connection = connections.find((c) => c.id === connectionId);

    if (!connection)
      return res.status(404).json({ message: "Connection not found" });

    connection.retainHistory = req.body.retainHistory;
    res.json({ message: "Chat history retention preference updated" });
  }
);

const PORT = 3000;
app.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
});
