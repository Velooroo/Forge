import { createBrowserRouter } from "react-router";
import { AuthPage } from "./pages/Auth";

import { AuthContext } from "./context/AuthContext";

export const router = createBrowserRouter([
  // Добавляю проверку на авторизацию пользователя

  {
    path: "/",
    element: <AuthPage />,
  },
]);

export default router;
