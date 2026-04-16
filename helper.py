import sys

def print_256_colors():
    print("Таблица 256 цветов ANSI:\n")
    
    # Выводим первые 16 цветов (стандартные и яркие)
    print("Стандартные (0-15):")
    for i in range(16):
        # \033[38;5;Nm — код для цвета текста (38;5; — спец. префикс)
        # \033[48;5;Nm — код для цвета фона
        sys.stdout.write(f"\033[48;5;{i}m {i:3} \033[0m")
        if (i + 1) % 8 == 0:
            print()
    print()
    # Основной блок (16-231): Цветовой куб 6x6x6
    print("Цветовой куб (16-231):")
    for i in range(16, 232):
        sys.stdout.write(f"\033[48;5;{i}m {i:3} \033[0m")
        if (i - 15) % 6 == 0:
            sys.stdout.write(" ")
        if (i - 15) % 36 == 0:
            print()
    print()

    # Оттенки серого (232-255)
    print("Оттенки серого (232-255):")
    for i in range(232, 256):
        sys.stdout.write(f"\033[48;5;{i}m {i:3} \033[0m")
    print("\n")

if __name__ == "__main__":
    print_256_colors()
