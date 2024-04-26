from nixpacks_python import detect, plan, build

def main():
    print(detect(path="../"))
    print(plan(path="../../web"))
    print(build(path="../../web", name="web1", ))

if __name__ == "__main__":
    main()