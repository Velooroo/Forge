import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Cpu, Github, ArrowRight, Sparkles } from "lucide-react";
import { useState } from "react";
import { login } from "@/api/auth";

export function AuthPage() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  // const handleAuth = (user: string, password: string) => {
  //   console.log(user, password);
  // };

  const handleAuth = (email: string, password: string) => {
    login(email, password);
  };

  return (
    <div className="min-h-screen bg-[#030607] text-white flex items-center justify-center relative overflow-hidden selection:bg-emerald-500/30 font-sans">
      {/* AURORA / GLOW BACKGROUND */}
      <div className="absolute inset-0 z-0 overflow-hidden">
        <motion.div
          animate={{
            scale: [1, 1.2, 1],
            opacity: [0.18, 0.32, 0.18],
            x: [0, 50, 0],
            y: [0, -30, 0],
          }}
          transition={{ duration: 10, repeat: Infinity, ease: "easeInOut" }}
          className="absolute top-[-15%] left-[-15%] w-[900px] h-[900px] bg-emerald-500/20 rounded-full blur-[140px]"
        />
        <motion.div
          animate={{
            scale: [1, 1.15, 1],
            opacity: [0.16, 0.34, 0.16],
            x: [0, -40, 0],
            y: [0, 40, 0],
          }}
          transition={{
            duration: 12,
            repeat: Infinity,
            ease: "easeInOut",
            delay: 1,
          }}
          className="absolute bottom-[-15%] right-[-15%] w-[900px] h-[900px] bg-cyan-500/18 rounded-full blur-[140px]"
        />
        <div className="absolute top-[25%] right-[35%] w-[520px] h-[520px] bg-blue-500/10 rounded-full blur-[140px]" />
      </div>

      {/* SUBTLE GRID (masked) */}
      <div className="absolute inset-0 z-0 bg-[linear-gradient(rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.03)_1px,transparent_1px)] bg-[size:72px_72px] [mask-image:radial-gradient(ellipse_65%_60%_at_50%_45%,black,transparent)]" />

      <div className="w-full max-w-6xl grid lg:grid-cols-2 gap-20 z-10 px-6">
        {/* LEFT */}
        <div className="hidden lg:flex flex-col justify-center space-y-10">
          <motion.div
            initial={{ opacity: 0, y: 18 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7 }}
          >
            <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-white/10 bg-white/5 backdrop-blur text-xs font-medium text-white/80 mb-6">
              <Sparkles className="w-3 h-3 text-emerald-300" />
              <span>System Online</span>
              <span className="ml-1 inline-flex h-2 w-2 rounded-full bg-emerald-400 shadow-[0_0_14px_rgba(52,211,153,0.8)]" />
            </div>

            <h1 className="text-6xl font-bold tracking-tight leading-[1.05]">
              Deployment{" "}
              <span className="text-transparent bg-clip-text bg-gradient-to-r from-emerald-300 via-cyan-300 to-blue-300">
                for hardware.
              </span>
            </h1>

            <p className="mt-6 text-lg text-white/55 leading-relaxed max-w-md">
              Forge is your Git control plane with Spark integration for fleets.
              Soft UI, hard infrastructure.
            </p>
          </motion.div>

          <div className="grid grid-cols-2 gap-4 max-w-md">
            {[
              { label: "Uptime", val: "99.99%" },
              { label: "Deploy", val: "< 50ms" },
            ].map((item, i) => (
              <motion.div
                key={item.label}
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.2 + i * 0.12, duration: 0.5 }}
                className="p-4 rounded-2xl bg-white/5 border border-white/10 backdrop-blur-md"
              >
                <div className="text-white/45 text-sm">{item.label}</div>
                <div className="text-xl font-semibold mt-1">{item.val}</div>
              </motion.div>
            ))}
          </div>
        </div>

        {/* RIGHT */}
        <div className="flex items-center justify-center">
          <motion.div
            initial={{ opacity: 0, scale: 0.96 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.55, delay: 0.1 }}
            className="w-full max-w-[420px] relative group"
          >
            {/* BORDER GLOW (emerald -> cyan -> blue) */}
            <div className="absolute -inset-[1px] rounded-3xl opacity-70 blur-md group-hover:opacity-100 transition duration-700" />

            {/* INNER GLASS CARD */}
            <div className="relative rounded-3xl p-8 border border-white/10 bg-white/[0.06] backdrop-blur-xl shadow-2xl">
              <div className="flex flex-col items-center mb-8">
                <div
                  className="w-12 h-12 rounded-2xl flex items-center justify-center mb-4
                  bg-gradient-to-tr from-emerald-400/80 to-cyan-400/80 shadow-lg shadow-emerald-500/15"
                >
                  <Cpu className="text-white/80 w-6 h-6" />
                </div>
                <h2 className="text-2xl font-bold">Welcome Back</h2>
                <p className="text-white/45 text-sm mt-2">
                  Sign in to access your console
                </p>
              </div>

              <form className="space-y-5">
                <div className="space-y-2">
                  <Label className="text-xs font-medium text-white/60 ml-1">
                    Email
                  </Label>
                  <Input
                    placeholder="you@forge.dev"
                    className="h-12 rounded-2xl bg-white/5 border-white/10 text-white placeholder:text-white/25
                      focus:bg-white/10 focus:border-emerald-400/40 focus:ring-emerald-500/20 transition-all"
                    onChange={(e) => {
                      setEmail(e.target.value);
                    }}
                  />
                </div>

                <div className="space-y-2">
                  <Label className="text-xs font-medium text-white/60 ml-1">
                    Password
                  </Label>
                  <Input
                    type="password"
                    placeholder="••••••••"
                    className="h-12 rounded-2xl bg-white/5 border-white/10 text-white placeholder:text-white/25
                      focus:bg-white/10 focus:border-emerald-400/40 focus:ring-emerald-500/20 transition-all"
                    onChange={(e) => {
                      setPassword(e.target.value);
                    }}
                  />
                </div>

                {/* BUTTON (no pink) */}
                <Button
                  className="w-full h-12 rounded-2xl font-semibold transition-all hover:scale-[1.015]
                    bg-gradient-to-r from-emerald-500 to-cyan-500 text-white/80
                    shadow-lg shadow-emerald-500/15 hover:shadow-emerald-500/25"
                  onClick={() => {
                    handleAuth(email, password);
                  }}
                >
                  Sign In
                  <ArrowRight className="w-4 h-4 ml-2" />
                </Button>
              </form>

              <div className="mt-8 pt-6 border-t border-white/10">
                <Button
                  variant="outline"
                  className="w-full h-12 rounded-2xl border-white/10 bg-white/5 hover:bg-white/10 text-white/80 hover:text-white transition-all"
                >
                  <Github className="w-4 h-4 mr-2" />
                  Continue with GitHub
                </Button>
              </div>

              <div className="mt-6 text-center">
                <p className="text-xs text-white/30">
                  By signing in, you agree to our{" "}
                  <a
                    href="#"
                    className="text-white/60 hover:text-white underline"
                  >
                    Terms
                  </a>
                </p>
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </div>
  );
}
