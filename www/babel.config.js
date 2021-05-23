const plugins = [];
if (process.env.ISTANBUL === "istanbul") {
  plugins.push([
    "babel-plugin-istanbul",
    {
      // specify some options for NYC instrumentation here
      // like tell it to instrument both JavaScript and Vue files
      extension: [".js", ".vue"],
    },
  ]);
}

module.exports = {
  // presets: ["@vue/cli-plugin-babel/preset"]
  presets: ["@vue/app"],
  plugins,
};
