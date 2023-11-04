import os
for file in os.listdir(os.path.dirname(__file__)):
    if file.endswith(".rs"):
        file_name = file[:-3]
        print(file_name)
        old_path = os.path.join(os.path.dirname(__file__), file)
        new_path = os.path.join(os.path.dirname(__file__), "Compiled", file_name)
        os.system(f"rustc -o {new_path} {old_path}")