import { createAlovaMockAdapter } from "@alova/mock";
import GlobalFetch from "alova/GlobalFetch";
import score from "@/mock/score";

const tauriMockAdapter = createAlovaMockAdapter([score], {
  enable: true,
  httpAdapter: GlobalFetch(),
  delay: 300,
  mockRequestLogger: true,
});

export default tauriMockAdapter;
